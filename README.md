# Quests Tracker: Crabby's Final Project 📜

Hey adventurers! Welcome to **Crabby's Quests Tracker**, the ultimate tool to keep tabs on all your epic adventures. 🌟 Whether you're rallying your squad, tracking loot, or planning your next epic quest.

this project’s got your back. Let's make managing quests as legendary as Crabby's journey itself! 🦀💥

## 🚀 What is Quests Tracker?

This is a project to help you stay organized with quests, adventurers, and their outcomes! Track who’s in, who’s out, and what went down, all while keeping things clean with **Rust** and **Domain-Driven Design** (DDD). It’s not just another tracker, it's Crabby's way to squad up and roll out! ⚔️✨

### 🔍 Key Features

- **QuestOps**: Manage all the quest operations. ⚔️
- **Quest Viewing**: Easily view all active quests, their details, and progress. 📝✨
- **Journey Ledger**: Keep a log of every quest journey, completed, failed, or in progress. 📖💫
- **Crew Switchboard**: Let's adventures to join or leave a quest. 🛠️👥
- **Adventures**: Just an adventure. 🌟
- **Guild Commanders**: Assign guild commanders to do the quest ops duty. 🏰🦸‍♂️
- **Authentication**: Keep everything secure, only the right adventurers can join the quests. 🔒✨

## 🛠️ Tech Stack

- **Rust Tokio Axum** 🦀: For maximum safety and async vibes.
- **PostgreSQL** 🐘: To store all that legendary loot and quest data.
- **Diesel ORM** 🚂: For smooth database ops.

## 📋 Project Structure

Organized with **Domain-Driven Design** + **Clean Architecture** to keep things tidy:

```text
quests_tracker/
├───config/
├───src/
│   ├───application/
│   │   └───usecases/
│   ├───domain/
│   │   ├───entities/
│   │   ├───repositories/
│   │   └───value_objects/
│   └───infrastructure
│       ├───axum_http/
│       └───postgres/
│           ├───migrations/
│           └───repositories/
├── Cargo.toml
```

## 🔧 How to Set Up

Ready to level up? Follow these steps:

1. **Clone the Repo**:

   ```sh
   git clone https://github.com/Rayato159/quests-tracker.git
   cd quests-tracker
   ```

2. **Install Dependencies**:
   Make sure you have **Rust** and **PostgreSQL** installed.
3. **Set Up Diesel**:

   ```sh
   cargo install diesel_cli --no-default-features --features postgres
   ```

4. **Run Migrations**:

   ```sh
   diesel setup
   diesel migration run
   ```

5. **Run the Server**:
   ```sh
   cargo run
   ```

Boom, you're in! 🎉

## 🧪 Testing

1. **Run Tests**:

   ```sh
   cargo test
   ```

2. **Runing Test Coverage**:

   ```sh
   cargo install cargo-tarpaulin
   ```

   ```sh
   cargo tarpaulin --out xml
   ```

## 📦 Upgrade All Dependecies

```sh
cargo update
```

## 🤖 Let's Cargo Clippy Refactor Your Code in Automatically

```sh
cargo clippy --fix --lib -p quests_tracker
```

## 📝 ENV Example

```text
STAGE=Local

SERVER_PORT=8080
SERVER_BODY_LIMIT=10 # MB
SERVER_TIMEOUT=90 # seconds

DATABASE_URL=postgres://postgres:123456@localhost/quests_tracker_db

JWT_ADVENTURER_SECRET=a_supersecret
JWT_ADVENTURER_REFRESH_SECRET=ar_supersecretrefresh
JWT_GUILD_COMMANDER_SECRET=g_supersecret
JWT_GUILD_COMMANDER_REFRESH_SECRET = gr_supersecretrefresh
```

## 🐳 Build

1. **Build Docker Image**:

   ```sh
   docker build -t quests-tracker:v1.0.0 -f ./Dockerfile .
   ```

2. **Run Docker Container**:

   ```sh
   podman run --name quests-tracker -p 8080:8080 \
    -e STAGE=Local \
    -e SERVER_PORT=8080 \
    -e SERVER_BODY_LIMIT=10 \
    -e SERVER_TIMEOUT=90 \
    -e DATABASE_URL=postgres://user:password@host:port/database_name \
    -e JWT_ADVENTURER_SECRET=xxxxx \
    -e JWT_ADVENTURER_REFRESH_SECRET=xxxxx \
    -e JWT_GUILD_COMMANDER_SECRET=xxxxx \
    -e JWT_GUILD_COMMANDER_REFRESH_SECRET=xxxxx \
    -d quests-tracker:v1.0.0
   ```

## ⚡️ Usage

- **Do a Quest Ops**: Start a new adventure for your crew.
- **Add Adventurers**: Let your squad join in on the action.
- **Track Progress**: Update and check the status of quests, know who’s thriving and who’s not.

## 🌍 Contributing

Feel like adding your magic to Crabby's adventure? Fork the repo, make those changes, and open a pull request. Crabby loves help from fellow adventurers! 💪✨

## 📜 License

This project is open-sourced under the **MIT License**. feel free to use, modify, and share.

## 🦀 Shoutout to Crabby!

Special shoutout to **Crabby**, the ultimate adventure guide. Without Crabby, none of these epic quests would be possible. Keep on adventuring! 🦀🚀

**Happy Questing, Adventurers!** 🌟
