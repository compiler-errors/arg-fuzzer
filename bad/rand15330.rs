
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15330(_: S1, _: S8) {}
        
        fn test15330() { foo15330(S5, S3, S2, S6, S1, S0); }
    