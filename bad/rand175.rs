
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo175(_: S2, _: S2, _: S0, _: S4) {}
        
        fn test175() { foo175(S6, S4, S5, S3, S7, S2); }
    