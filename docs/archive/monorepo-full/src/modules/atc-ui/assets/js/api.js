// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
/**
 * A-TownChain OS — Frontend API Client v2.0.0
 * Spricht ausschließlich mit dem Gateway (Port 4000).
 */

const GATEWAY = window.GATEWAY_URL || "http://localhost:4000";
const API_KEY  = localStorage.getItem("atc_api_key") || "atc-dev-key-2026";

const headers = () => ({
    "Content-Type": "application/json",
    "X-API-Key": API_KEY,
});

// ── Generic Request ─────────────────────────────────
async function request(method, path, body = null) {
    const opts = { method, headers: headers() };
    if (body) opts.body = JSON.stringify(body);
    try {
        const resp = await fetch(`${GATEWAY}${path}`, opts);
        const data = await resp.json();
        if (!resp.ok) throw new Error(data.error || `HTTP ${resp.status}`);
        return data;
    } catch (e) {
        console.error(`[API] ${method} ${path}:`, e.message);
        throw e;
    }
}

// ── Blockchain ──────────────────────────────────────
const Chain = {
    status:    () => request("GET",  "/api/blockchain/status"),
    blocks:    (n=10) => request("GET", `/api/blockchain/blocks?limit=${n}`),
    tx:        (hash) => request("GET", `/api/blockchain/tx/${hash}`),
    sendTx:    (tx)   => request("POST", "/api/blockchain/send", tx),
};

// ── Wallet ──────────────────────────────────────────
const Wallet = {
    balance:   (addr) => request("GET", `/api/wallet/balance/${addr}`),
    create:    ()     => request("POST", "/api/wallet/create"),
    send:      (from, to, amount, key) =>
                         request("POST", "/api/wallet/send", {from, to, amount, key}),
    history:   (addr) => request("GET", `/api/wallet/history/${addr}`),
};

// ── Shivamon ────────────────────────────────────────
const Shivamon = {
    list:      (addr) => request("GET", `/api/game/shivamon?owner=${addr}`),
    get:       (id)   => request("GET", `/api/game/shivamon/${id}`),
    battle:    (a, b) => request("POST", "/api/game/battle", {attacker: a, defender: b}),
    breed:     (a, b) => request("POST", "/api/game/breed",  {parent1: a, parent2: b}),
    marketplace: {
        list:  ()        => request("GET",  "/api/marketplace/listings"),
        buy:   (id, addr)=> request("POST", "/api/marketplace/buy",  {listing_id: id, buyer: addr}),
        sell:  (id, price)=> request("POST", "/api/marketplace/list", {nft_id: id, price}),
    },
};

// ── Governance ──────────────────────────────────────
const Gov = {
    proposals: ()         => request("GET",  "/api/governance/proposals"),
    propose:   (d)        => request("POST", "/api/governance/propose", d),
    vote:      (id, v, a) => request("POST", `/api/governance/vote/${id}`, {voter: a, vote: v}),
    results:   (id)       => request("GET",  `/api/governance/results/${id}`),
};

// ── Nodes ───────────────────────────────────────────
const Nodes = {
    list:   () => request("GET", "/api/nodes"),
    status: () => request("GET", "/api/nodes/status"),
    join:   (d) => request("POST", "/api/nodes/join", d),
};

// ── AI ──────────────────────────────────────────────
const AI = {
    chat:    (msg) => request("POST", "/api/ai/chat", {message: msg}),
    analyze: (tx)  => request("POST", "/api/ai/analyze", {tx}),
};

// ── Franchise ───────────────────────────────────────
const Franchise = {
    list:   ()    => request("GET",  "/api/franchise/"),
    get:    (id)  => request("GET",  `/api/franchise/${id}`),
    create: (d)   => request("POST", "/api/franchise/create", d),
    join:   (id, member, stake) =>
                     request("POST", `/api/franchise/${id}/join`, {member, stake}),
    stats:  ()    => request("GET",  "/api/franchise/stats"),
};

// ── Health ──────────────────────────────────────────
const Gateway = {
    health: () => request("GET", "/health"),
    status: () => request("GET", "/api/status"),
};

// ── Export ──────────────────────────────────────────
window.ATC = { Chain, Wallet, Shivamon, Gov, Nodes, AI, Franchise, Gateway };
console.log("[ATC API] v2.0.0 geladen | Gateway:", GATEWAY);
