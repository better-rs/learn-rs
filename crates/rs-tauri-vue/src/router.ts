import {createRouter, createWebHistory, RouterView} from 'vue-router'
import Home from './views/Home.vue'

import NotFound from './views/NotFound.vue'


const component = () => {
    console.log('fetching component')
    return import('./views/Generic.vue')
}


import LongView from './views/HomeView.vue'


import {globalState} from './store'
import {scrollWaiter} from './scrollWaiter'



let removeRoute: (() => void) | undefined

export const routerHistory = createWebHistory()
export const router = createRouter({
    history: routerHistory,
    strict: true,
    routes: [
        {path: '/home', redirect: '/'},
        {
            path: '/',
            components: {default: Home, other: component},
            props: {default: to => ({waited: to.meta.waitedFor})},
        },
        {
            path: '/always-redirect',
            redirect: () => ({
                name: 'user',
                params: {id: String(Math.round(Math.random() * 100))},
            }),
        },


        {path: encodeURI('/n/€'), name: 'euro', component},
        {path: '/n/:n', name: 'increment', component},
        {path: '/multiple/:a/:b', name: 'multiple', component},
        {path: '/long-:n', name: 'long', component: LongView},
        {
            path: '/lazy',
            meta: {transition: 'slide-left'},
            component: async () => {
                await delay(500)
                return component
            },
        },
        {
            path: '/with-guard/:n',
            name: 'guarded',
            component,
            beforeEnter(to) {
                if (to.params.n !== 'valid') return false
            },
        },




        {path: '/:data(.*)', component: NotFound, name: 'NotFound'},


        {
            path: '/admin',
            children: [
                {path: '', component},
                {path: 'dashboard', component},
                {path: 'settings', component},
            ],
        },
    ],
    async scrollBehavior(to, from, savedPosition) {
        await scrollWaiter.wait()
        if (savedPosition) {
            return savedPosition
        } else {
            if (to.matched.every((record, i) => from.matched[i] !== record))
                return {left: 0, top: 0}
        }
        // leave scroll as it is by not returning anything
        // https://github.com/Microsoft/TypeScript/issues/18319
        return false
    },
})

// router.push({ name: 'user', params: {} })

const delay = (t: number) => new Promise(resolve => setTimeout(resolve, t))

// remove trailing slashes
router.beforeEach(to => {
    if (/.\/$/.test(to.path)) {
        to.meta.redirectCode = 301
        return to.path.replace(/\/$/, '')
    }
})

router.beforeEach(async to => {
    // console.log(`Guard from ${from.fullPath} to ${to.fullPath}`)
    if (to.params.id === 'no-name') return false

    const time = Number(to.query.delay)
    if (time > 0) {
        console.log('⏳ waiting ' + time + 'ms')
        to.meta.waitedFor = time
        await delay(time)
    }
})

router.beforeEach(() => {
    if (globalState.cancelNextNavigation) return false
})

router.afterEach((to, from) => {
    if (to.name === from.name && to.name === 'repeat') {
        const toDepth = to.path.split('/').length
        const fromDepth = from.path.split('/').length
        to.meta.transition = toDepth < fromDepth ? 'slide-right' : 'slide-left'
    }
})

router.afterEach((to, from) => {
    // console.log(
    //   `After guard: from ${from.fullPath} to ${
    //     to.fullPath
    //   } | location = ${location.href.replace(location.origin, '')}`
    // )
})

export function go(delta: number) {
    return new Promise((resolve, reject) => {
        function popStateListener() {
            clearTimeout(timeout)
        }

        window.addEventListener('popstate', popStateListener)

        function clearHooks() {
            removeAfterEach()
            removeOnError()
            window.removeEventListener('popstate', popStateListener)
        }

        // if the popstate event is not called, consider this a failure
        const timeout = setTimeout(() => {
            clearHooks()
            reject(new Error('Failed to use router.go()'))
            // using 0 leads to false positives
        }, 1)

        const removeAfterEach = router.afterEach((_to, _from, failure) => {
            clearHooks()
            resolve(failure)
        })
        const removeOnError = router.onError(err => {
            clearHooks()
            reject(err)
        })

        router.go(delta)
    })
}

// @ts-expect-error
window._go = go

router.beforeEach(to => {
    // console.log('second guard')
    if (typeof to.query.to === 'string' && to.query.to) return to.query.to
})

const dirLog = {
    '': '？',
    back: '⏪',
    forward: '⏩',
}
routerHistory.listen((to, from, info) => {
    console.log(`${dirLog[info.direction]} as a ${info.type}`)
})
