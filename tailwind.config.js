module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        extend: {
            colors: {
                'gray-850': '#1a1a1a',
            }
        },
        screens: {
            'desktop': '1426px',
            'mobile': {max: '1024px'},
            'tablet': {max: '1425px'}
        },
        fontFamily: {
            brand: ['Inter', 'Roboto Mono']
        },
    },
    variants: {},
    plugins: [],
};
