
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11492(_: S1, _: S2, _: S4, _: S7, _: S8) {}
        
        fn test11492() { foo11492(S6, S6, S2, S6, S1, S1, S0); }
    