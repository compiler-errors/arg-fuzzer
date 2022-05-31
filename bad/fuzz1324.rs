
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1324(_: S3, _: S1, _: S8) {}
        
        fn test1324() { foo1324(S2, S2, S1, S7, S1, S4); }
    