import { ZerisClient } from '@zeris/sdk';
import { Connection, clusterApiUrl, Keypair } from '@solana/web3.js';
import * as fs from 'fs';

async function main() {
    console.log('Zeris Node.js TypeScript Example');

    // Setup connection
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    // Load or generate keypair
    let keypair: Keypair;
    try {
        const keypairData = JSON.parse(fs.readFileSync('keypair.json', 'utf8'));
        keypair = Keypair.fromSecretKey(new Uint8Array(keypairData));
    } catch {
        console.log('Generating new keypair...');
        keypair = Keypair.generate();
        fs.writeFileSync('keypair.json', JSON.stringify(Array.from(keypair.secretKey)));
        console.log('Keypair saved to keypair.json');
    }

    console.log('Public key:', keypair.publicKey.toString());

    // Initialize client
    const client = new ZerisClient(connection);
    const proofBuilder = client.getProofBuilder();

    // Mock proof data (in real usage, load from snarkjs output)
    const mockProofData = {
        proof: {
            pi_a: ['123', '456', '1'],
            pi_b: [['789', '012'], ['345', '678'], ['1', '0']],
            pi_c: ['901', '234', '1'],
            protocol: 'groth16',
            curve: 'bn128'
        },
        publicSignals: ['5', '3', '15'] // a * b = c
    };

    try {
        console.log('Building proof data...');
        const proofData = proofBuilder.buildProofFromJson(mockProofData);

        console.log('Submitting proof...');
        const signature = await client.submitProof(keypair, proofData);
        console.log('✅ Proof submitted successfully!');
        console.log('Transaction signature:', signature);

        // Wait for confirmation
        await connection.confirmTransaction(signature, 'confirmed');
        console.log('✅ Transaction confirmed');

        // Get updated state
        console.log('Fetching current state...');
        const state = await client.getZerisState();
        console.log('📊 Current state:');
        console.log('   Last Proof Hash:', Buffer.from(state.lastProofHash).toString('hex'));
        console.log('   Merkle Root:', Buffer.from(state.merkleRoot).toString('hex'));
        console.log('   Proof Count:', state.proofCount);
        console.log('   Authority:', state.authority.toString());

        // Listen for events (for 10 seconds)
        console.log('Listening for events...');
        await client.listenForEvents((event) => {
            console.log('📡 Event:', event);
        });

        // Wait a bit
        await new Promise(resolve => setTimeout(resolve, 10000));

    } catch (error) {
        console.error('❌ Error:', error);
        process.exit(1);
    }
}

main().catch(console.error);