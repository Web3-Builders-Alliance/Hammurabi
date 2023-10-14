# 🌍 Imagine a World with Better Decentralized Exchanges 🌍

What if everyone could access a decentralized exchange (DEX) that's swift, secure, and intuitive? What if liquidity providers didn't have to hemorrhage billions annually? That's the vision we're turning into reality at **Hammurabi**.

---

## 🚀 Introducing Hammurabi: A Paradigm Shift in Cryptocurrency Trading 🚀

Hammurabi aims to change the dynamics of cryptocurrency trading and our mission is:

_Liquid token swapping and abundant on-chain liquidity stand as the pillars of DeFi and numerous crypto applications. However, if liquidity providers persistently face losses, it could jeopardize and potentially halt a broad spectrum of crypto-economic activities. Therefore, our paramount mission is to protect and empower liquidity providers._

To fullfill our mission we focus on 4 different pillar:

- 💡 **Beginner-Friendly**: An intuitive UI makes it a breeze to locate assets and place trades.

- 🥱 **For "Lazy" Liquidity Provider**: Our cutting-edge matching algorithm rejuvenates pool rebalancing with a dynamic fee system. The result? A "Lazy mode" for liquidity providers to minimize loss-prone rebalancing and boost fee generation.
  
- 🌐 **Permissionless Interoperable LPs**: Move your LP tokens wherever you want, and capitalize on strategy stacking. It's a game-changer for LPs, unlocking avenues for maximum fee generation.

- 🔄 **Universal Trading**: Trade any cryptocurrency, on any chain, with anyone globally, minus the third-party trust issues.

---

## 🛠 Current Functionalities available for HYPERDRIVE HACKATHON:

- **Curve Library Built in wasm**: For all our calculations, we've crafted an interoperable curve that functions both in Rust and in Typescript using wasm. This interoperability is crucial as it enables smooth integration between contract and frontend/server/bots. Tasks such as routing, slippage, swap, and fee calculations can now be effortlessly executed off-chain. Just query our PDAs to fetch the latest pool data.

- **Dynamic Fee via Curve Library**: Our system is ingeniously devised to magnify returns for passive liquidity providers and simultaneously ensure conducive trading conditions. How do we achieve this balance? Trades that aid in pool rebalancing are given discounted fees, while those which might disturb market-making are levied with higher fees. This dual-action system provides a robust defense against market volatility.

- **Dynamic Deposit Function [_WIP - reference L0STE branch_]**: This functionality enables LPs to deposit in a pool that aligns closely with current trading conditions. It simplifies the rebalancing of your position and decelerates trading in out-of-balance pools (Such pools still process trades but with heftier fees, ensuring increased revenue for LPs without jeopardizing their positions excessively).

- **Testing Alogrithm for Other DeFi Strategies**: We created a Tool that other DeFi Protocol can use to test their strategies. Our Algorithm analyze both Arbitrage Opportunity, by referencing a "Cex Price" calulated leveraging a Binomial Price Model that i speedy, dependable and not data-heavy, and User Habits, by mirroring user trades by sourcing them from Price Aggregator like Jup.Ag and mimicking what would happen in the real world by acting only on the best price through all the compared strategies. 
    **[_Our current WIP data analysis]**: 
    - **Markout Calculation** on every current amm or customized strategies
    - **Strategy Testing Against Current Market Standard** like fixed fee pool, ...
    - **Resistance Against LVR Test**: calculate how much money your strategy loose against arbitrageurs 
