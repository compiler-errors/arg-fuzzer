
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo125(_: S8, _: S1, _: S3, _: S5, _: S4, _: S2, _: S7, _: S6) {}
        
        fn test125() { foo125(S0, S6, S4, S1, S5, S5); }
    