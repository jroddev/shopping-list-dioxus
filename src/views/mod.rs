//! The views module contains the components for all Layouts and Routes for our shopping list app.
//! Each layout and route in our [`Route`] enum will render one of these components.

mod shopping_lists_page;
pub use shopping_lists_page::ShoppingListsPage;

mod item_listing_page;
pub use item_listing_page::ItemListingPage;
