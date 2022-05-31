
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12297(_: S1, _: S2, _: S8) {}
        
        fn test12297() { foo12297(S6, S0, S1, S7, S5); }
    