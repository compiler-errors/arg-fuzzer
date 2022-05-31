
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1933(_: S7, _: S2, _: S6, _: S4, _: S7) {}
        
        fn test1933() { foo1933(S1, S2, S3, S4, S6, S7, S8); }
    