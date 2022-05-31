
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11936(_: S2, _: S2, _: S6, _: S3, _: S2) {}
        
        fn test11936() { foo11936(S1, S2, S3, S5, S6, S7, S8); }
    