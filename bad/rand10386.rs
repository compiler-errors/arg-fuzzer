
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10386(_: S5, _: S8) {}
        
        fn test10386() { foo10386(S3, S5, S1); }
    