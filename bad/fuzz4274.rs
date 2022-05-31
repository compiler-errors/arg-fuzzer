
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4274(_: S4, _: S7) {}
        
        fn test4274() { foo4274(S1, S1, S8, S1, S2, S4, S3, S8); }
    