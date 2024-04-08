import { useState, useEffect, useCallback, FC, useMemo ,forwardRef,useImperativeHandle} from 'react';
import Link from 'next/link'
import { useRouter } from 'next/router';
import { hasLogin, getAddress, RainbowKitKey } from '../utils/util';
import React from 'react';
import Image from 'next/image';
import { WalletMultiButton, WalletDisconnectButton, WalletConnectButton } from '@solana/wallet-adapter-react-ui'
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js'
import { Program, AnchorProvider, web3, utils } from '@coral-xyz/anchor';
import styles from '../styles/Header.module.css'
import idl from '../idl.json';
import { PROGRAM_ID, PREFLIGHTCOMMITMENT, MINT_SEEDs, TOKEN_METADATA_PROGRAMID ,API_URL} from '../constant/const'

const myLoader = ({ src, width, quality }) => {
  return `${src}?w=${width}&q=${quality || 75}`
}

const { SystemProgram, Keypair } = web3;
const baseAccount = Keypair.generate();
const programID = new PublicKey(PROGRAM_ID);//new PublicKey('672VeCnpAxhkd9TueCo8653hiYXP8kJRXbhPzi3YWZor') 

function Header({ source, cart, onDelete }: any,ref) {

  const [showCartDialog, setShowCartDialog] = useState(false)
  const [totalPrice, setTotalPrice] = useState(0)
  const [cartList, setCartList] = useState([])
  const [account, setAccount] = useState({})
  const router = useRouter()
  const { connection: isConnected } = useConnection();
  const { publicKey: address, sendTransaction, connected } = useWallet();

  //初始化--start
  const network = WalletAdapterNetwork.Devnet;
  const wallet = useWallet();
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  async function getProvider() {
    const connection = new Connection(endpoint, PREFLIGHTCOMMITMENT);
    console.log('connection::::', connection)
    const provider = new AnchorProvider(
      connection, wallet, PREFLIGHTCOMMITMENT,
    );
    return {
      connection,
      provider
    };
  }

  async function initialize() {
    const { provider, connection } = await getProvider();
    console.log('provider:::::', provider)
    const program = new Program(idl, programID, provider);
    //payer
    const userKey = new web3.PublicKey(address?.toBase58());
    window.pg = {
      provider,
      program,
      connection,
      programID,
      userKey
    }
    console.log('program:::::', program)
  }

  useEffect(() => {
    console.log('初始化钱包', connected)
    connected && window.location.pathname === '/pay' && initialize()
  }, [connected, address])

  //初始化--end


  useEffect(() => {
    if (!isConnected || !address?.toBase58()) {
      localStorage.removeItem('accountInfo')
      localStorage.removeItem(RainbowKitKey)
      // localStorage.setItem('autoConnect',false)
      setAccount({})
      if(window.location.pathname !== '/'){
        // router.push('/')
      }
      return
    }
    let accountInfo = localStorage.getItem('accountInfo') as any
    if (accountInfo) {
      accountInfo = JSON.parse(accountInfo) || {}
    } else {
      accountInfo = {}
    }
    if (accountInfo.userAddress && accountInfo.userAddress != 'null') {
      setAccount(accountInfo)
      localStorage.setItem(RainbowKitKey, `${accountInfo.userAddress}`)
      localStorage.setItem('autoConnect', true)
      return
    }
    fetchLogin()
  }, [address, isConnected])

  const fetchLogin = ()=>{
    fetch(`${API_URL}/login`, {
      method: 'POST',
      body: JSON.stringify({
        userAddress: address?.toBase58(),
        token: address?.toBase58()
      }),
      headers: {
        'content-type': 'application/json'
      }
    }).then(response => {
      return response.json()
    }).then(res => {
      if (res.code == 0) {
        setAccount(res.data)
        localStorage.setItem('accountInfo', JSON.stringify(res.data))
        localStorage.setItem(RainbowKitKey, `${address}`)
        localStorage.setItem('autoConnect', true)
      }
    })
  }

  useImperativeHandle(ref, () => ({
    fetchLogin: fetchLogin
  }));


  useEffect(() => {
    let list = localStorage.getItem('cartList') as any
    if (list) {
      list = JSON.parse(list)
    } else {
      list = []
    }
    let num = 0
    list.map(item => {
      num += item.price * item.num
    })
    setCartList(list)
    setTotalPrice(Math.round(num * 1000000000) / 1000000000)
  }, [cart])

  const onOpenDialog = async () => {
    if (!source) return
    setShowCartDialog(!showCartDialog)
  }

  const onCloseDialog = () => {
    setShowCartDialog(false)
  }

  const onToPay = () => {
    if (!hasLogin()) {
      alert('please connect wallet!')
      return
    }
    if (!cartList.length) {
      alert('please add cart!')
      return
    }
    console.log('等待中....')
    router.push('/pay')
  }

  const onDisconnect = (e) => {
    localStorage.removeItem('accountInfo')
    localStorage.removeItem(RainbowKitKey)
    // localStorage.setItem('autoConnect',false)
    setAccount({})
  }


  return (
    <div className={styles.header}>
      <div className={styles.left}><Link href={{ pathname: '/' }}><img src="https://raw.githubusercontent.com/1va7/BlockTargeter/main/Logo%20color.png" alt="" /></Link></div>
      <div className={styles.right}>
        <div className={styles.connect}>
          <WalletMultiButton style={{ background: '#8846D6', height: '40px', lineHeight: '40px', color: '#fff' }} />
          {/* {
            account.userAddress?<WalletDisconnectButton onClick={onDisconnect}/>:<WalletMultiButton />
          } */}
          {
            account.userAddress ? <div className={styles.account}>
              {
                account?.assets?.map(item => {
                  return <span key={item.asset}>
                    {item.quantity} {item.asset}
                  </span>
                })
              }
            </div> : ''
          }
        </div>
        <div className={styles.cart} onClick={onOpenDialog} id='shopCart'>
          <img className={styles['cart-icon']} src='https://www.shilingou.cn:9801/temp/image_2.png' />
          <span className={styles['cart-num']}>({cartList.length})</span>
          {
            source && <div className={styles['cart-down']}>
              <div className={styles['cart-title']}>
                <div>My Cart ({cartList.length})</div>
                {/* <span onClick={onCloseDialog}>X</span> */}
              </div>
              <div className={styles['cart-list']}>
                {
                  cartList.length ? <>
                    {
                      cartList.map(item => {
                        return (
                          <div className={styles['cart-item']} key={item.NFTId}>
                            <Image src={item.URL} alt="" width={160} height={160} loader={myLoader} />
                            <div className={styles['cart-item-content']}>
                              <div className={styles['name']}>{item.name}</div>
                              <div className={styles['sub-name']}>{item.description}</div>
                              <div className={styles['price']}>{item.price} SOL</div>
                            </div>
                            <div className={styles['cart-delete']}>
                              <span className={styles['cart-item-num']}>x{item.num}</span>
                              <i onClick={() => { onDelete(item.NFTId) }} className='iconfont icon-shanchuanniu'></i>
                            </div>
                          </div>
                        )
                      })
                    }
                  </> : <div className={styles.no_data}><i className='iconfont icon-gouwuchehuanshikongde'></i> <div>No Data</div></div>
                }

              </div>
              <div className={styles['total']}>Total: {totalPrice} SOL</div>
              <div className={styles['cart-footer']}>
                <div className={styles['footer-btn']} id='pay' onClick={onToPay}>Continue to payment</div>
              </div>
            </div>
          }
        </div>
      </div>

    </div>
  );
}

export default forwardRef(Header);
