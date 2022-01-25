CREATE TABLE workouts (
    id VARCHAR NOT NULL PRIMARY KEY,
    tag VARCHAR NOT NULL,
    week INTEGER NOT NULL,
    workout_type VARCHAR NOT NULL,
    description TEXT NOT NULL,
    distance TEXT NOT NULL
  )