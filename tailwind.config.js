module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        screens: {
            'desktop': '1025px',
            'mobile': {max: '1024px'}
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
