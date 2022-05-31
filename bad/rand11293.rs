
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11293(_: S0, _: S4, _: S1, _: S4, _: S6) {}
        
        fn test11293() { foo11293(S1, S2, S8, S6, S7, S5, S3, S4); }
    