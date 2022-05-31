
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4048(_: S6, _: S2, _: S6) {}
        
        fn test4048() { foo4048(S5, S1, S4, S6, S7); }
    