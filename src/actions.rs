pub mod actions {
    pub trait PerformAction {
        fn perform(&self);
    }

    //------------------------------

    pub struct AddRecordAction {
        name: String,
    }

    impl AddRecordAction {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl PerformAction for AddRecordAction {
        fn perform(&self) {
            println!("Record {} added.", &self.name);
        }
    }

    // --------------------

    pub struct RmRecordAction {
        name: String,
    }

    impl RmRecordAction {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl PerformAction for RmRecordAction {
        fn perform(&self) {
            println!("Record {} removed.", &self.name);
        }
    }

    // --------------------

    pub struct PrintHelp;

    impl PerformAction for PrintHelp {

        fn perform(&self) {
            println!("Help:");
        }
    }

    pub struct ListRecords;

    impl PerformAction for ListRecords {
        fn perform(&self) {
            println!("Records:");
        }
    }

    // --------------------

    pub enum AppAction {
        PrintHelp(PrintHelp),
        ListRecords(ListRecords),
        AddRecord(AddRecordAction),
        RmRecord(RmRecordAction),
    }
}