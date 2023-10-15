# 🌍 Imagine a DeFi protocol that put LPs as the focus 🌍

In the intricate dance of DeFi, liquidity providers (LPs) should be the lead. Why should they stay in the shadows, suffering losses, when they play a pivotal role? **Hammurabi** lights up the stage for LPs, ensuring they not only shine but also thrive.


## 🚀 Introducing Hammurabi: The Vanguard of Liquity Revolution 🚀

Hammurabi's vision is not about creating just another DeFi platform. It's about rewriting the rules, placing LPs at the heart of it all:

_Liquid token swapping and abundant on-chain liquidity stand as the pillars of DeFi and numerous crypto applications. However, if liquidity providers persistently face losses, it could jeopardize and potentially halt a broad spectrum of crypto-economic activities. Therefore, our paramount mission is to protect and empower liquidity providers._

**Translating this vision into reality: Hammurabi is built upon four game-changing pillars:**

- 💡 **Beginner-Friendly**: An intuitive UI makes it a breeze to locate assets and place trades.

- 🥱 **For "Lazy" Liquidity Provider**: Our cutting-edge matching algorithm rejuvenates pool rebalancing with a dynamic fee system. The result? A "Lazy mode" for liquidity providers to minimize loss-prone rebalancing and boost fee generation.
  
- 🌐 **Permissionless Interoperable LPs**: Move your LP tokens wherever you want, and capitalize on strategy stacking. It's a game-changer for LPs, unlocking avenues for maximum fee generation.

- 💬 **Social Vault**: Revolutionizing SocialFi, we provide traders a playground to devise, test, and refine their strategies. Once proven, these strategies are opened up to our user base, ensuring a community-driven, optimized trading experience.


## 🛠 Current Functionalities available for HYPERDRIVE HACKATHON:

- **Curve Library Built in wasm**: For all our calculations, we've crafted an interoperable curve that functions both in Rust and in Typescript using wasm. This interoperability is crucial as it enables smooth integration between contract and frontend/server/bots. Tasks such as routing, slippage, swap, and fee calculations can now be effortlessly executed off-chain. Just query our PDAs to fetch the latest pool data. [Find out more HERE] (https://github.com/deanmlittle/constant-product-curve)

- **Dynamic Fee via Curve Library**: Our system is ingeniously devised to magnify returns for passive liquidity providers and simultaneously ensure conducive trading conditions. How do we achieve this balance? Trades that aid in pool rebalancing are given discounted fees, while those which might disturb market-making are levied with higher fees. This dual-action system provides a robust defense against market volatility. 

- **Dynamic Deposit Function [_WIP - reference L0STE branch_]**: This functionality enables LPs to deposit in a pool that aligns closely with current trading conditions. It simplifies the rebalancing of your position and decelerates trading in out-of-balance pools (Such pools still process trades but with heftier fees, ensuring increased revenue for LPs without jeopardizing their positions excessively).

- **Testing Alogrithm for Other DeFi Strategies**: We created a Tool that other DeFi Protocol can use to test their strategies. Our Algorithm analyze both Arbitrage Opportunity, by referencing a "Cex Price" calulated leveraging a Binomial Price Model that i speedy, dependable and not data-heavy, and User Habits, by mirroring user trades by sourcing them from Price Aggregator like Jup.Ag and mimicking what would happen in the real world by acting only on the best price through all the compared strategies. Currently we're wokring on shipping the 3 first functionalities of our Analysis Alogrithm:
    - **Markout Calculation** on every current amm or customized strategies
    - **Strategy Testing Against Current Market Standard** like fixed fee pool, ...
    - **Resistance Against LVR Test**: calculate how much money your strategy loose against arbitrageurs 
[Find out more HERE] (https://github.com/ASCorreia/Hamm-Algo)