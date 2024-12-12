
import React, { useState } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

const App = () => {
    return (
        <div>
            <h1>Solana Staking Application</h1>
            <WalletMultiButton />
        </div>
    );
};

export default App;
