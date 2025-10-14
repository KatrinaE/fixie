// Message Type Modules
// Organized by category and FIX specification sections

// Infrastructure and Session Messages
pub mod infrastructure;
pub mod business_message_rejects;
pub mod application_sequencing;
pub mod network_status;
pub mod user_management;

// Trading Messages
pub mod single_general_order_handling;
pub mod order_mass_handling;
pub mod order_cross_handling;
pub mod confirmations;
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

// Post-Trade Messages
pub mod account_reporting;       // [SECTION 700]
pub mod position_maintenance;    // [SECTION 710]
pub mod allocation;              // [SECTION 720]
pub mod confirmation;            // [SECTION 730]
pub mod settlement_instruction;  // [SECTION 740]
pub mod trade_capture_reporting; // [SECTION 750]

// Re-exports
pub use infrastructure::*;
pub use business_message_rejects::*;
pub use application_sequencing::*;
pub use network_status::*;
pub use user_management::*;
pub use single_general_order_handling::*;
pub use order_mass_handling::*;
pub use order_cross_handling::*;
pub use confirmations::*;
pub use mass_orders::*;
pub use multileg_orders::*;
pub use program_trading::*;
pub use indication::*;
pub use event_communication::*;
pub use quotation::*;
pub use market_data::*;
pub use market_structure::*;
pub use securities_reference::*;
pub use account_reporting::*;
pub use position_maintenance::*;
pub use allocation::*;
pub use confirmation::*;
pub use settlement_instruction::*;
pub use trade_capture_reporting::*;
