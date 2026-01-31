# Graph-minton Roadmap

## Project Overview
Graph-minton is a Rust application that parses badminton game data and player aliases, normalizes the data, and integrates it with a Helix graph database to build a network of player relationships and match history.

---

## Completed Work ✅

### Data Parsing & Loading
- ✅ Load all alias files from `data/aliases/`
- ✅ Load all game CSV files from `data/scores/`
- ✅ Handle parsing errors gracefully with error reporting
- ✅ Normalize game data using alias mappings

### Core Infrastructure
- ✅ Data models: `Game`, `Player`
- ✅ Request structures for database operations: `CreatePlayerRequest`, `CreateWithRequest`, `CreateAgainstRequest`
- ✅ Database schema and query definitions in Helix

---

## Remaining Work 🚧

### 1. Database Integration
- [ ] Implement player node insertion
- [ ] Implement "With" relationship insertion (teammates)
- [ ] Implement "Against" relationship insertion (opponents)

### 2. Game Processing Pipeline
- [ ] Convert normalized games into database operations
- [ ] Batch insert games into the database
- [ ] Handle duplicate detection (same game loaded twice)

### 3. Query Implementation
- [ ] Implement player statistics queries (games played, win/loss)
- [ ] Implement partnership analysis (most common teammates)
- [ ] Implement opponent analysis (head-to-head records)
- [ ] Implement player performance over time
- [ ] Implement team composition analysis

### 4. Data Validation
- [ ] Validate game data before insertion
- [ ] Check for inconsistent player aliases
- [ ] Detect and report data anomalies
- [ ] Add config for subs vs non-subs analysis

### 5. Documentation
- [ ] Architecture documentation
- [ ] Data model diagrams
