module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        screens: {
            'desktop': '1426px',
            'mobile': {max: '1024px'},
            'tablet': {max: '1425px'}
        },
        fontFamily: {
            brand: ['Inter', 'Roboto Mono']
        },
        colors: {
            'gray-850': '#1a1a1a'
        }
    },
    variants: {},
    plugins: [],
};
