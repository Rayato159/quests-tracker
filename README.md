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
│   │   ├───services/
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

## ⚡️ Usage

- **Create a Quest**: Start a new adventure for your crew.
- **Add Adventurers**: Let your squad join in on the action.
- **Track Progress**: Update and check the status of quests, know who’s thriving and who’s not.

## 🌍 Contributing

Feel like adding your magic to Crabby's adventure? Fork the repo, make those changes, and open a pull request. Crabby loves help from fellow adventurers! 💪✨

## 📜 License

This project is open-sourced under the **MIT License**. feel free to use, modify, and share.

## 🦀 Shoutout to Crabby!

Special shoutout to **Crabby**, the ultimate adventure guide. Without Crabby, none of these epic quests would be possible. Keep on adventuring! 🦀🚀

**Happy Questing, Adventurers!** 🌟
