
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6547(_: S5, _: S6, _: S8) {}
        
        fn test6547() { foo6547(S6, S6, S5, S0); }
    