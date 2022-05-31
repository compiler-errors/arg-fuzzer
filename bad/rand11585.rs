
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11585(_: S5, _: S4, _: S7, _: S7, _: S2) {}
        
        fn test11585() { foo11585(S3, S8, S5, S4, S6, S1, S2); }
    