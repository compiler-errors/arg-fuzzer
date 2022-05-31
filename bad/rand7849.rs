
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7849(_: S1, _: S4, _: S7, _: S8) {}
        
        fn test7849() { foo7849(S6, S1, S7, S5, S5, S7); }
    