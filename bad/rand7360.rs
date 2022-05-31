
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7360(_: S0, _: S5, _: S1, _: S1) {}
        
        fn test7360() { foo7360(S1, S2, S3, S4, S5, S6, S7, S8); }
    