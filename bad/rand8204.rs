
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8204(_: S1, _: S5, _: S1, _: S5, _: S2) {}
        
        fn test8204() { foo8204(S1, S4, S3, S0, S5, S4, S5); }
    