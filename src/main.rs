use std::collections::HashMap;

#[derive(Debug)]
struct TradePlan {
    stock: String,
    current_holding: i32,
    target_holding: i32,
    difference: i32,
    action: String,
    trade_amount: i32,
}

fn calculate_trade_plan(current_holdings: &HashMap<String, i32>, target_holdings: &HashMap<String, i32>) -> Vec<TradePlan> {
    let mut all_stocks: Vec<String> = current_holdings.keys().chain(target_holdings.keys()).cloned().collect();
    all_stocks.sort_unstable();
    all_stocks.dedup();

    let mut trade_plans: Vec<TradePlan> = all_stocks.into_iter()
        .map(|stock| {
            let current = current_holdings.get(&stock).copied().unwrap_or(0);
            let target = target_holdings.get(&stock).copied().unwrap_or(0);
            let difference = target - current;

            let action = match difference.cmp(&0) {
                std::cmp::Ordering::Greater => "买入",
                std::cmp::Ordering::Less => "卖出",
                std::cmp::Ordering::Equal => "不变",
            }.to_string();

            TradePlan {
                stock,
                current_holding: current,
                target_holding: target,
                difference,
                action,
                trade_amount: difference.abs(),
            }
        })
        .filter(|plan| plan.action != "不变")
        .collect();

    trade_plans.sort_unstable_by(|a, b| {
        b.action.cmp(&a.action)
            .then_with(|| b.trade_amount.cmp(&a.trade_amount))
    });

    trade_plans
}

fn main() {
    let current_holdings: HashMap<String, i32> = [
        ("AAPL", 100),
        ("GOOGL", 50),
        ("MSFT", 75),
    ].iter().map(|&(k, v)| (k.to_string(), v)).collect();

    let target_holdings: HashMap<String, i32> = [
        ("AAPL", 150),
        ("GOOGL", 0),
        ("MSFT", 100),
        ("AMZN", 25),
    ].iter().map(|&(k, v)| (k.to_string(), v)).collect();

    let plan = calculate_trade_plan(&current_holdings, &target_holdings);

    println!("{:<10} {:<15} {:<15} {:<10} {:<10} {:<10}", "股票", "当前持仓", "目标持仓", "差额", "操作", "交易数量");
    for trade in plan {
        println!("{:<10} {:<15} {:<15} {:<10} {:<10} {:<10}",
                 trade.stock, trade.current_holding, trade.target_holding,
                 trade.difference, trade.action, trade.trade_amount);

    }
}