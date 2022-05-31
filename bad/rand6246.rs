
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6246(_: S4, _: S3, _: S6, _: S0, _: S2) {}
        
        fn test6246() { foo6246(S5, S4, S5, S7, S0, S0); }
    