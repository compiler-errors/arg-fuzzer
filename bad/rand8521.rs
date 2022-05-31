
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8521(_: S1, _: S6) {}
        
        fn test8521() { foo8521(S2, S4, S0, S4, S5, S1, S0); }
    