
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo316(_: S3, _: S4, _: S7) {}
        
        fn test316() { foo316(S1, S1, S4, S4, S2, S3, S4); }
    