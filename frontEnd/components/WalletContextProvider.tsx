import { FC, ReactNode ,useMemo,useCallback, useEffect} from 'react';
import { ConnectionProvider, WalletProvider,useWallet } from '@solana/wallet-adapter-react'
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import {clusterApiUrl,Connection, PublicKey } from '@solana/web3.js'
import { WalletAdapterNetwork,Adapter, WalletError } from '@solana/wallet-adapter-base';
import {PhantomWalletAdapter} from '@solana/wallet-adapter-wallets';
import { createDefaultAuthorizationResultCache, SolanaMobileWalletAdapter } from '@solana-mobile/wallet-adapter-mobile';
import { SnackbarProvider, useSnackbar } from 'notistack';
import { type SolanaSignInInput } from '@solana/wallet-standard-features';
import { verifySignIn } from '@solana/wallet-standard-util';
import { AutoConnectProvider, useAutoConnect } from './AutoConnectProvider';
import { Program, Provider, web3 } from '@project-serum/anchor';
require('@solana/wallet-adapter-react-ui/styles.css');

const WalletContextProvider: FC<{ children: ReactNode }> = ({ children }) => {
	const { autoConnect } = useAutoConnect();

	const network = WalletAdapterNetwork.Devnet;

    const endpoint = useMemo(() => clusterApiUrl(network), [network]);
    const wallets = useMemo(
        () => [
            // new SolanaMobileWalletAdapter({
            //     appIdentity: { name: 'Metana Demo' },
            //     authorizationResultCache: createDefaultAuthorizationResultCache(),
            // }),
            new PhantomWalletAdapter()
        ],
        [network]
    );

	const { enqueueSnackbar } = useSnackbar();
    const onError = useCallback(
        (error: WalletError, adapter?: Adapter) => {
            enqueueSnackbar(error.message ? `${error.name}: ${error.message}` : error.name, { variant: 'error' });
            console.error(error, adapter);
        },
        [enqueueSnackbar]
    );

    const autoSignIn = useCallback(async (adapter: Adapter) => {
        if (!('signIn' in adapter)) return true;
        const input: SolanaSignInInput = {
            domain: window.location.host,
            address: adapter.publicKey ? adapter.publicKey.toBase58() : undefined,
            statement: 'Please sign in.',
        };
        const output = await adapter.signIn(input);

        if (!verifySignIn(input, output)) throw new Error('Sign In verification failed!');

        return false;
    }, []);

	return (
		<AutoConnectProvider>
		<ConnectionProvider endpoint={endpoint}>
	    <WalletProvider wallets={wallets}  autoConnect>
	      <WalletModalProvider>
	        { children }
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
	</AutoConnectProvider>
	)
}

export default WalletContextProvider