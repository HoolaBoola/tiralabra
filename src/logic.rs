pub mod calculator;

mod shunting_yard;

// So that it's possible to just
// `use ...::shunting_yard;`
// instead of
// `use ...::shunting_yard::shunting_yard`;
use shunting_yard::shunting_yard;
