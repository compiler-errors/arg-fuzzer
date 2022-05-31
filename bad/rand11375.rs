
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11375(_: S3, _: S4, _: S6) {}
        
        fn test11375() { foo11375(S8, S3, S7, S4, S2); }
    