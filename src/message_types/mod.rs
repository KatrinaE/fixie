// Message Type Modules
// Organized by category and FIX specification sections

// Infrastructure and Session Messages
pub mod application_sequencing;
pub mod network_status;
pub mod user_management;

// Trading Messages
pub mod single_general_order_handling;
pub mod order_mass_handling;
pub mod order_cross_handling;
pub mod mass_orders;
pub mod multileg_orders;
pub mod program_trading;

// Pre-Trade Messages
pub mod indication;              // [SECTION 100]
pub mod event_communication;     // [SECTION 200]
pub mod quotation;               // [SECTION 300]
pub mod market_data;             // [SECTION 400]
pub mod market_structure;        // [SECTION 500]
pub mod securities_reference;    // [SECTION 600]

// Re-exports
pub use application_sequencing::*;
pub use network_status::*;
pub use user_management::*;
pub use single_general_order_handling::*;
pub use order_mass_handling::*;
pub use order_cross_handling::*;
pub use mass_orders::*;
pub use multileg_orders::*;
pub use program_trading::*;
pub use indication::*;
pub use event_communication::*;
pub use quotation::*;
pub use market_data::*;
pub use market_structure::*;
pub use securities_reference::*;
