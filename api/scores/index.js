// In-memory storage for scores (resets on server restart)
// For persistent storage, connect to Azure Cosmos DB or Table Storage
let scores = [
    { name: "PAC", score: 5000 },
    { name: "MAN", score: 4000 },
    { name: "RUST", score: 3000 },
    { name: "WASM", score: 2000 },
    { name: "AZURE", score: 1000 }
];

module.exports = async function (context, req) {
    context.log('Processing score request.');

    if (req.method === "GET") {
        // Return top 10 scores
        context.res = {
            body: scores
        };
    } else if (req.method === "POST") {
        // Add new score
        const { name, score } = req.body;

        if (!name || typeof score !== 'number') {
            context.res = {
                status: 400,
                body: "Invalid input. Name and score (number) are required."
            };
            return;
        }

        scores.push({ name, score });
        
        // Sort descending and keep top 10
        scores.sort((a, b) => b.score - a.score);
        if (scores.length > 10) {
            scores = scores.slice(0, 10);
        }

        context.res = {
            body: scores
        };
    }
}
