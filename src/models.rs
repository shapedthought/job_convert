// vbr get and post structs
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbrGetJob {
    pub description: String,
    pub guest_processing: GuestProcessing,
    pub id: String,
    pub is_disabled: bool,
    pub is_high_priority: bool,
    pub name: String,
    pub schedule: Schedule,
    pub storage: Storage,
    #[serde(rename = "type")]
    pub type_field: String,
    pub virtual_machines: VirtualMachines,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbrPostJob {
    pub description: String,
    pub guest_processing: GuestProcessing,
    pub is_disabled: bool,
    pub is_high_priority: bool,
    pub name: String,
    pub schedule: Schedule,
    pub storage: Storage,
    #[serde(rename = "type")]
    pub type_field: String,
    pub virtual_machines: VirtualMachines2,
}

// this is a thing of beauty
impl From<VbrGetJob> for VbrPostJob {
    fn from(job: VbrGetJob) -> Self {
        let inc2 = job
            .virtual_machines
            .includes
            .iter()
            .map(|i| Include2 {
                host_name: i.inventory_object.host_name.clone(),
                name: i.inventory_object.name.clone(),
                object_id: i.inventory_object.object_id.clone(),
                type_field: i.inventory_object.type_field.clone(),
            })
            .collect::<Vec<Include2>>();

        let vm = VirtualMachines2 {
            excludes: job.virtual_machines.excludes,
            includes: inc2,
        };

        VbrPostJob {
            description: job.description,
            guest_processing: job.guest_processing,
            is_disabled: job.is_disabled,
            is_high_priority: job.is_high_priority,
            name: job.name,
            schedule: job.schedule,
            storage: job.storage,
            type_field: job.type_field,
            virtual_machines: vm,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestProcessing {
    pub app_aware_processing: AppAwareProcessing,
    pub guest_credentials: GuestCredentials,
    #[serde(rename = "guestFSIndexing")]
    pub guest_fsindexing: GuestFsindexing,
    pub guest_interaction_proxies: GuestInteractionProxies,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppAwareProcessing {
    pub app_settings: Vec<Value>,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestCredentials {
    pub credentials_per_machine: Vec<Value>,
    pub creds_id: String,
    pub creds_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestFsindexing {
    pub indexing_settings: Vec<Value>,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestInteractionProxies {
    pub auto_selection: bool,
    pub proxy_ids: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub after_this_job: AfterThisJob,
    pub backup_window: BackupWindow,
    pub continuously: Continuously,
    pub daily: Daily,
    pub monthly: Monthly,
    pub periodically: Periodically,
    pub retry: Retry,
    pub run_automatically: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AfterThisJob {
    pub is_enabled: bool,
    pub job_name: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWindow {
    pub backup_window: BackupWindow2,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWindow2 {
    pub days: Vec<Day>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub day: String,
    pub hours: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Continuously {
    pub backup_window: BackupWindow3,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWindow3 {
    pub days: Vec<Day2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day2 {
    pub day: String,
    pub hours: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daily {
    pub daily_kind: String,
    pub days: Vec<String>,
    pub is_enabled: bool,
    pub local_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monthly {
    pub day_number_in_month: String,
    pub day_of_month: i64,
    pub day_of_week: String,
    pub is_enabled: bool,
    pub local_time: String,
    pub months: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Periodically {
    pub backup_window: BackupWindow4,
    pub frequency: i64,
    pub is_enabled: bool,
    pub periodically_kind: String,
    pub start_time_within_an_hour: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWindow4 {
    pub days: Vec<Day3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day3 {
    pub day: String,
    pub hours: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retry {
    pub await_minutes: i64,
    pub is_enabled: bool,
    pub retry_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    pub advanced_settings: AdvancedSettings,
    pub backup_proxies: BackupProxies,
    pub backup_repository_id: String,
    pub gfs_policy: GfsPolicy,
    pub retention_policy: RetentionPolicy,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedSettings {
    pub active_fulls: ActiveFulls,
    pub backup_health: BackupHealth,
    pub backup_mode_type: String,
    pub full_backup_maintenance: FullBackupMaintenance,
    pub notifications: Notifications,
    pub scripts: Scripts,
    pub storage_data: StorageData,
    pub storage_integration: StorageIntegration,
    pub synthentic_fulls: SynthenticFulls,
    pub v_sphere: VSphere,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveFulls {
    pub is_enabled: bool,
    pub monthly: Monthly2,
    pub weekly: Weekly,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monthly2 {
    pub day_number_in_month: String,
    pub day_of_months: i64,
    pub day_of_week: String,
    pub is_enabled: bool,
    pub months: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weekly {
    pub days: Vec<String>,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupHealth {
    pub is_enabled: bool,
    pub monthly: Monthly3,
    pub weekly: Weekly2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monthly3 {
    pub day_number_in_month: String,
    pub day_of_months: i64,
    pub day_of_week: String,
    pub is_enabled: bool,
    pub months: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weekly2 {
    pub days: Vec<String>,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullBackupMaintenance {
    #[serde(rename = "RemoveData")]
    pub remove_data: RemoveData,
    pub defragment_and_compact: DefragmentAndCompact,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveData {
    pub after_days: i64,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefragmentAndCompact {
    pub is_enabled: bool,
    pub monthly: Monthly4,
    pub weekly: Weekly3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monthly4 {
    pub day_number_in_month: String,
    pub day_of_months: i64,
    pub day_of_week: String,
    pub is_enabled: bool,
    pub months: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weekly3 {
    pub days: Vec<String>,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notifications {
    pub email_notifications: EmailNotifications,
    #[serde(rename = "sendSNMPNotifications")]
    pub send_snmpnotifications: bool,
    pub vm_attribute: VmAttribute,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailNotifications {
    pub custom_notification_settings: Value,
    pub is_enabled: bool,
    pub notification_type: Value,
    pub recipients: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VmAttribute {
    pub append_to_exisiting_value: bool,
    pub is_enabled: bool,
    pub notes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scripts {
    pub day_of_week: Vec<String>,
    pub periodicity_type: String,
    pub post_command: PostCommand,
    pub pre_command: PreCommand,
    pub run_script_every: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCommand {
    pub command: String,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreCommand {
    pub command: String,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageData {
    pub compression_level: String,
    pub enable_inline_data_dedup: bool,
    pub encryption: Encryption,
    pub exclude_deleted_file_blocks: bool,
    pub exclude_swap_file_blocks: bool,
    pub storage_optimization: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    pub encryption_password_id_or_null: String,
    pub encryption_password_tag: Value,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageIntegration {
    pub failover_to_standard_backup: bool,
    pub is_enabled: bool,
    pub limit_processed_vm: bool,
    pub limit_processed_vm_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynthenticFulls {
    pub is_enabled: bool,
    pub monthly: Monthly5,
    pub weekly: Weekly4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monthly5 {
    pub day_number_in_month: String,
    pub day_of_months: i64,
    pub day_of_week: String,
    pub is_enabled: bool,
    pub months: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weekly4 {
    pub days: Vec<String>,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VSphere {
    pub changed_block_tracking: ChangedBlockTracking,
    #[serde(rename = "enableVMWareToolsQuiescence")]
    pub enable_vmware_tools_quiescence: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangedBlockTracking {
    #[serde(rename = "enableCBTautomatically")]
    pub enable_cbtautomatically: bool,
    pub is_enabled: bool,
    #[serde(rename = "resetCBTonActiveFull")]
    pub reset_cbton_active_full: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupProxies {
    pub auto_selection: bool,
    pub proxy_ids: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GfsPolicy {
    pub is_enabled: bool,
    pub monthly: Monthly6,
    pub weekly: Weekly5,
    pub yearly: Yearly,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monthly6 {
    pub desired_time: String,
    pub is_enabled: bool,
    pub keep_for_number_of_months: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weekly5 {
    pub desired_time: String,
    pub is_enabled: bool,
    pub keep_for_number_of_weeks: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yearly {
    pub desired_time: String,
    pub is_enabled: bool,
    pub keep_for_number_of_years: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionPolicy {
    pub quantity: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachines {
    pub excludes: Excludes,
    pub includes: Vec<Include>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachines2 {
    pub excludes: Excludes,
    pub includes: Vec<Include2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Excludes {
    pub disks: Vec<Disk>,
    pub templates: Templates,
    pub vms: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    pub disks: Vec<Value>,
    pub disks_to_process: String,
    #[serde(rename = "removeFromVMConfiguration")]
    pub remove_from_vmconfiguration: bool,
    pub vm_object: VmObject,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VmObject {
    pub host_name: String,
    pub name: String,
    pub object_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Templates {
    pub exclude_from_incremental: bool,
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Include {
    pub inventory_object: InventoryObject,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Include2 {
    pub host_name: String,
    pub name: String,
    pub object_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryObject {
    pub host_name: String,
    pub name: String,
    pub object_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
