class GameGuessModel {
  constructor() {
    this.validationRules = {
      cardId: { type: 'number', min: 1, required: true },
      position: { type: 'number', min: -1, max: 6, required: true },
      roundNumber: { type: 'number', min: 1, required: true }
    };
  }

  serializeGuessRequest(cardId, position, roundNumber) {
    const data = {
      cardId: parseInt(cardId),
      position: parseInt(position),
      roundNumber: parseInt(roundNumber)
    };

    return data;
  }
}

export default GameGuessModel;