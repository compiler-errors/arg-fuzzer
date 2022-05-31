
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo218(_: S1, _: S4, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test218() { foo218(S3, S5, S1, S0, S1, S5, S6); }
    