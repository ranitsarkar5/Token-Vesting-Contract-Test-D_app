import React, { useState, useEffect } from 'react';
import './index.css';

const TARGET_AMOUNT = 5000;

function App() {
  const [address, setAddress] = useState('');
  const [pledged, setPledged] = useState(0);
  const [inputAmount, setInputAmount] = useState('');
  const [feedback, setFeedback] = useState(null);

  useEffect(() => {
    // 2. Load cached total from localStorage (Performance caching requirement)
    const cached = localStorage.getItem('campaign_pledged');
    if (cached) {
      setPledged(parseInt(cached, 10));
    }
  }, []);

  const connectWallet = async () => {
    setFeedback({ type: 'loading', msg: 'Prompting Wallet...' });
    setTimeout(() => {
      const mockKey = "GBMOCK" + Math.random().toString(36).substring(2, 10).toUpperCase() + "WALLETFORTESTING";
      setAddress(mockKey);
      setFeedback({ type: 'success', msg: 'Connected (Demo Mode)!' });
      setTimeout(() => setFeedback(null), 3000);
    }, 1000);
  };

  const handlePledge = async () => {
    const amount = parseInt(inputAmount, 10);
    if (!amount || amount <= 0) {
      setFeedback({ type: 'error', msg: 'Please enter a valid amount.' });
      return;
    }

    setFeedback({ type: 'loading', msg: 'Submitting transaction...' });

    // Simulate transaction delay
    setTimeout(() => {
      const newTotal = pledged + amount;
      setPledged(newTotal);
      localStorage.setItem('campaign_pledged', newTotal.toString()); // Cache update
      setInputAmount('');
      setFeedback({ type: 'success', msg: `Successfully pledged ${amount} XLM!` });
      setTimeout(() => setFeedback(null), 5000);
    }, 2000);
  };

  const progress = Math.min((pledged / TARGET_AMOUNT) * 100, 100);

  return (
    <div className="app-container">
      <header>
        <h1>SorobanFundMe</h1>
        <p>Decentralized Crowdfunding on Stellar</p>
      </header>

      {!address ? (
        <button className="connect-btn" onClick={connectWallet}>
          Connect Freighter Wallet
        </button>
      ) : (
        <div className="campaign-card">
          <div className="stat-row">
            <span className="stat-label">Connected As</span>
            <span className="stat-label" style={{ fontFamily: 'monospace' }}>{address.substring(0, 6)}...{address.substring(50)}</span>
          </div>

          <div className="stat-row" style={{ marginTop: '24px' }}>
            <span className="stat-label">Target Goal</span>
            <span className="stat-value">{TARGET_AMOUNT} XLM</span>
          </div>

          <div className="stat-row">
            <span className="stat-label">Total Pledged</span>
            <span className="stat-value" style={{ color: progress >= 100 ? 'var(--success)' : 'inherit' }}>
              {pledged} XLM
            </span>
          </div>

          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${progress}%` }}></div>
          </div>
        </div>
      )}

      {address && (
        <div className="action-area">
          <div className="input-group">
            <input
              type="number"
              placeholder="Amount in XLM"
              value={inputAmount}
              onChange={(e) => setInputAmount(e.target.value)}
              disabled={progress >= 100}
            />
            <button onClick={handlePledge} disabled={progress >= 100 || !inputAmount}>
              {progress >= 100 ? 'Funded' : 'Pledge'}
            </button>
          </div>
        </div>
      )}

      {feedback && (
        <div className={`feedback ${feedback.type}`}>
          {feedback.msg}
        </div>
      )}
    </div>
  );
}

export default App;
