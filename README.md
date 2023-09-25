# Description

Project is a simple mock of [etherscan](https://etherscan.io/). The following features are supported. 

1. Transaction hash search
2. Display latest Ether price
3. Display total Ether count
4. Display total Ethereum node count

# Requirements
- rust
- cargo
- tailwind
- npm
- npx

### Install dependencies
```
cargo install --path .
```



# Run
### Development

1. Start Tailwind
```
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

2. Start web application on hot reload

```
dx serve --hot-reload
```

### Build

1. Generate CSS file
```
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

2. Build project
```
cargo build
```

## Sources

1. Etherscan APIs - https://docs.etherscan.io/api-endpoints/stats-1#get-total-supply-of-ether
2. Styling Template - https://www.creative-tim.com/learning-lab/tailwind-starter-kit/documentation/css
3. Tailwind CSS - tailwindcss.com
4. Dioxus Lab - https://dioxuslabs.com/learn/0.4/reference/user_input