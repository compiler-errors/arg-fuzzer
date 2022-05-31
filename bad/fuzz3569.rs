
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3569(_: S1, _: S5, _: S7, _: S6) {}
        
        fn test3569() { foo3569(S8, S8, S3, S7, S2); }
    