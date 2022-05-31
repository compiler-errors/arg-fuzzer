
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo566(_: S6, _: S6, _: S2, _: S0, _: S1) {}
        
        fn test566() { foo566(S7, S7, S6, S4, S6, S3, S4); }
    