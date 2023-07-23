use std::{collections::BTreeMap, marker::PhantomData};

use anyhow::bail;
use binread::BinReaderExt;

use crate::registry::{
    transactionlog::{ApplicationResult, TransactionLog, TransactionLogsEntry},
    Hive,
};

use super::{BaseBlock, CleanHive, DirtyHive, HiveBaseBlock};

pub trait ContainsHive<B>: Dissolve<B> + BaseBlock
where
    Self: Sized,
    B: BinReaderExt,
{
    fn with_transaction_log(self, log: TransactionLog) -> anyhow::Result<HiveWithLogs<B, Self>> {
        let mut transaction_logs = BTreeMap::new();
        let base_block = self.base_block().expect("hive has no base block");
        if !base_block.is_dirty() {
            /*
            If a hive isn't dirty, but a transaction log file (new format) contains subsequent log entries, they are ignored.

            <https://github.com/msuhanov/regf/blob/master/Windows%20registry%20file%20format%20specification.md#dirty-state-of-a-hive>
             */
            log::warn!("Hive is not dirty, no transaction log is needed. doing nothing...");
        } else {
            let primary_sequence_number = *base_block.primary_sequence_number();
            let secondary_sequence_number = *base_block.secondary_sequence_number();

            if primary_sequence_number != secondary_sequence_number + 1 {
                bail!("the difference between the sequence numbers is greater than 1: {primary_sequence_number} and {secondary_sequence_number}");
            }

            for entry in log {
                if *entry.sequence_number() > secondary_sequence_number {
                    transaction_logs.insert(*entry.sequence_number(), entry);
                }
            }
        }

        Ok(HiveWithLogs {
            hive: self,
            transaction_logs,
            phantom: PhantomData,
        })
    }
}

pub trait Dissolve<B>
where
    B: BinReaderExt,
{
    fn dissolve(self) -> (Hive<B, DirtyHive>, BTreeMap<u32, TransactionLogsEntry>);
}

pub struct HiveWithLogs<B, C>
where
    B: BinReaderExt,
    C: ContainsHive<B>,
{
    hive: C,
    transaction_logs: BTreeMap<u32, TransactionLogsEntry>,
    phantom: PhantomData<B>,
}

impl<B, C> ContainsHive<B> for HiveWithLogs<B, C>
where
    B: BinReaderExt,
    C: ContainsHive<B>,
{
}

impl<B, C> BaseBlock for HiveWithLogs<B, C>
where
    B: BinReaderExt,
    C: ContainsHive<B>,
{
    fn base_block(&self) -> Option<&HiveBaseBlock> {
        self.hive.base_block()
    }
}

impl<B, C> Dissolve<B> for HiveWithLogs<B, C>
where
    B: BinReaderExt,
    C: ContainsHive<B>,
{
    fn dissolve(self) -> (Hive<B, DirtyHive>, BTreeMap<u32, TransactionLogsEntry>) {
        let (hive, mut logs) = self.hive.dissolve();
        logs.extend(self.transaction_logs);
        (hive, logs)
    }
}

impl<B, C> HiveWithLogs<B, C>
where
    B: BinReaderExt,
    C: ContainsHive<B>,
{
    pub fn apply_logs(self) -> Hive<B, CleanHive> {
        let (mut hive, logs) = self.dissolve();

        log::info!("trying to apply {} entries: ", logs.len());

        for entry in logs.into_values() {
            log::info!(
                "found entry for sequence number {}",
                entry.sequence_number()
            );
            if ApplicationResult::Applied != hive.apply_transaction_log(entry) {
                break;
            }
        }
        hive.treat_hive_as_clean()
    }
}