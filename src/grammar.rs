/// This module defines the grammar and tokens for a custom language.
///
/// # Grammar
/// The grammar is defined as follows:
///
/// - `expression = Expr`
/// - `EmptyString = e`
/// - `InputSymbols = E`
/// - `Operators = Op`
///
/// ```
/// E => {
///     ( > ),
///     ( < ),
///     ( + ),
///     ( - ),
///     ( . ),
///     ( , ),
///     ( [] )
/// }
/// ```
///
/// - `Expr => { Loop, Op }`
/// - `Expr => e`
/// - `Loop => { "[" Expr "]" }`
/// - `Op   => { ">" | "<" | "+" | "-" | "." | "," }*`
///
/// # Tokens
/// Enum representing different token types.
///
/// ```
/// #[derive(Debug, Copy, Clone)]
/// pub enum Token {
///     MoveBack = '<' as isize,
///     MoveForward = '>' as isize,
///     Add = '+' as isize,
///     Sub = '-' as isize,
///     StdOut = '.' as isize,
///     StdIn = ',' as isize,
///     LoopStart = '[' as isize,
///     LoopEnd = ']' as isize,
/// }
/// ```
///
/// # Expressions
/// Enum representing different expression types.
///
/// ```
/// #[derive(Debug)]
/// pub enum Expression {
///     Loop(Vec<Expression>),
///     Operator(Box<Operators>),
/// }
/// ```
///
/// # Operator
/// Struct representing operators with type and count.
///
/// ```
/// #[derive(Debug)]
/// pub struct Operator {
///     pub _type_name: Token,
///     pub _count: u32,
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    MoveBack = '<' as isize,
    MoveForward = '>' as isize,
    Add = '+' as isize,
    Sub = '-' as isize,
    StdOut = '.' as isize,
    StdIn = ',' as isize,
    LoopStart = '[' as isize,
    LoopEnd = ']' as isize,
}

#[derive(Debug)]
pub enum Expression {
    Loop(Vec<Expression>),
    Operator(Box<Operator>),
}

#[derive(Debug)]
pub struct Operator {
    pub type_name: Token,
    pub count: usize,
}
