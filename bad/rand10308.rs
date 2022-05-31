
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10308(_: S3, _: S1, _: S6) {}
        
        fn test10308() { foo10308(S2, S3, S5, S7, S8); }
    