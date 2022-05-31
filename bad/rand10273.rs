
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10273(_: S1, _: S6, _: S8) {}
        
        fn test10273() { foo10273(S7, S6, S1, S2, S2, S0, S1); }
    