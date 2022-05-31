
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1718(_: S6, _: S4) {}
        
        fn test1718() { foo1718(S3, S6, S5, S2, S7, S8, S6, S8); }
    