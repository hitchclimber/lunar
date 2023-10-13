db = db.getSiblingDB('moonbatteries');
db.createCollection('moon');

db.moon.insertOne({ macAddress: 'INVALID_TESTER', lastConnected: new Date() });
